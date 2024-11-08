import { useEffect, useState } from "react";
import { useSession } from "./session.ts";

const API_URL = "http://127.0.0.1:3000";

export type DaDirectory = {
    sid: number;
    name: string;
    parent?: number;
    children: DaDirectory[];
};

export type PostDirectoryRequest = {
    name: string;
    parent?: number;
};

export type PutDirectoryRequest = {
    name: string;
};

export enum ApiErrorType {
    Forbidden,
    NotFound,
    InternalServerError,
}

export type ApiError = {
    kind: ApiErrorType;
    status: number;
    message: string;
};

class ApiClient {
    private userId: number;

    constructor(userId: number) {
        this.userId = Math.abs(userId);
    }

    public async fetchApi<T>(
        path: string,
        custom_options: RequestInit = {},
    ) {
        const response = await fetch(`${API_URL}${path}`, {
            headers: {
                "Content-Type": "application/json",
                "X-Entity-Uid": this.userId.toString(),
            },
            ...custom_options,
        });

        if (!response.ok) {
            let kind: ApiErrorType;
            switch (response.status) {
                case 403:
                    kind = ApiErrorType.Forbidden;
                    break;
                case 404:
                    kind = ApiErrorType.NotFound;
                    break;
                default:
                    kind = ApiErrorType.InternalServerError;
                    break;
            }

            throw {
                kind,
                status: response.status,
                message: response.statusText,
            } as ApiError;
        }
        if (response.headers.get("Content-Length") == "0") {
            return null;
        }
        return response.json() as T;
    }

    public getDirectory(id: number) {
        return this.fetchApi<DaDirectory>(`/directory/${id}`, {
            method: "GET",
        });
    }

    public postDirectory(request: PostDirectoryRequest) {
        return this.fetchApi<DaDirectory>(`/directory`, {
            method: "POST",
            body: JSON.stringify(request),
        });
    }

    public putDirectory(sid: number, request: PutDirectoryRequest) {
        return this.fetchApi<DaDirectory>(`/directory/${sid}`, {
            method: "PUT",
            body: JSON.stringify(request),
        });
    }

    public deleteDirectory(sid: number) {
        return this.fetchApi(`/directory/${sid}`, { method: "DELETE" });
    }
}

export function useApi(): ApiClient | null {
    const [userId] = useSession();
    const [apiClient, setApiClient] = useState<ApiClient | null>(null);

    useEffect(() => {
        if (userId != null) {
            setApiClient(new ApiClient(parseInt(userId)));
        } else {
            setApiClient(null);
        }
    }, [userId]);

    return apiClient;
}
