import React, { useEffect, useState } from "react";
import { Button, TreeView } from "../Components.ts";
import { useSession } from "../hooks/session.ts";
import { ApiError, ApiErrorType, DaDirectory, useApi } from "../hooks/api.ts";

type UnhandledError = {
    kind: ApiErrorType;
    prettyMessage?: string;
};

function DirectoryView() {
    const [_, setUserId] = useSession();
    const api = useApi();
    const [rootDirectory, setRootDirectory] = useState<DaDirectory | null>(
        null,
    );

    const [currentError, setCurrentError] = useState<UnhandledError | null>(
        null,
    );

    const handleLogout = () => {
        setUserId(null);
    };

    useEffect(() => {
        if (api == null) {
            return;
        }
        api.getDirectory(0)
            .then((directory: DaDirectory) => {
                setRootDirectory(directory);
            })
            .catch((reason: ApiError) => {
                if (reason.kind == ApiErrorType.NotFound) {
                    console.log("No root directory, creating ...");
                    api.postDirectory({
                        name: "Root",
                    }).then((directory: DaDirectory) => {
                        setRootDirectory(directory);
                    })
                        .catch((reason: ApiError) => {
                            setCurrentError({
                                kind: reason.kind,
                                prettyMessage:
                                    "Failed to create root directory",
                            });
                        });
                } else {
                    setCurrentError({
                        kind: reason.kind,
                        prettyMessage: "Unhandled error",
                    });
                }
            });
    }, [api]);

    return (
        <div className="flex min-h-full flex-col">
            <div className="justify-start">
                <Button onClick={handleLogout}>Logout</Button>
            </div>
            <div className="flex flex-row grow">
                {rootDirectory != null
                    ? <TreeView className="pt-4" directory={rootDirectory} />
                    : <div>Loading ...</div>}
            </div>
        </div>
    );
}

export default DirectoryView;
