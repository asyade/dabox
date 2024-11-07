import React, { useEffect, useState } from "react";
import Button from "../components/Button.tsx";
import { useSession } from "../hooks/session.ts";
import { ApiError, ApiErrorType, DaDirectory, useApi } from "../hooks/Api.ts";

function DirectoryView() {
    const [_, setUserId] = useSession();
    const [api] = useApi();
    const [rootDirectory, setRootDirectory] = useState<DaDirectory | null>(
        null,
    );

    const handleLogout = () => {
        setUserId(null);
    };

    useEffect(() => {
        if (api == null) {
            return;
        }
        console.log("Loading root directory ...");
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
                            console.error("Failed to create root directory !");
                            console.error(reason);
                        });
                } else {
                    console.error("Internal server error !");
                }
            });
    }, [api]);

    return (
        <div className="flex min-h-full flex-col">
            <div className="justify-start">
                <Button onClick={handleLogout}>Logout</Button>
            </div>
            <div className="flex flex-row grow">
                <h1>Directory</h1>
            </div>
        </div>
    );
}

export default DirectoryView;
