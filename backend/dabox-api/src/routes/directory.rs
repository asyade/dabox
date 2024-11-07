use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct PostDirectoryRequest {
    name: String,
    parent: Option<DaDirectorySid>,
}

#[derive(Debug, Deserialize)]
pub struct PutDirectoryRequest {
    name: String,
}

#[instrument(skip(repository))]
pub async fn get_directory<R: DaRepository + 'static>(
    Path(directory_sid): Path<DaDirectorySid>,
    user: ApiUser,
    repository: State<Arc<R>>,
) -> ApiResult<Json<DaDirectory>> {
    let directory = repository.get_directory(user.uid(), directory_sid).await?;
    Ok(Json(directory))
}

#[instrument(skip(repository))]
pub async fn post_directory<R: DaRepository + 'static>(
    user: ApiUser,
    repository: State<Arc<R>>,
    Json(request): Json<PostDirectoryRequest>,
) -> ApiResult<Json<DaDirectory>> {
    Ok(Json(
        repository
            .create_directory(user.uid(), &request.name, request.parent)
            .await?,
    ))
}

#[instrument(skip(repository))]
pub async fn put_directory<R: DaRepository + 'static>(
    user: ApiUser,
    repository: State<Arc<R>>,
    Path(directory_sid): Path<DaDirectorySid>,
    Json(request): Json<PutDirectoryRequest>,
) -> ApiResult<Json<DaDirectory>> {
    repository
        .rename_directory(user.uid(), directory_sid, &request.name)
        .await?;
    Ok(Json(
        repository.get_directory(user.uid(), directory_sid).await?,
    ))
}

#[instrument(skip(repository))]
pub async fn delete_directory<R: DaRepository + 'static>(
    Path(directory_sid): Path<DaDirectorySid>,
    user: ApiUser,
    repository: State<Arc<R>>,
) -> ApiResult<()> {
    repository
        .delete_directory(user.uid(), directory_sid)
        .await?;
    Ok(())
}
