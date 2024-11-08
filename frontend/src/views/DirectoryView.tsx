import React, { useEffect, useState } from "react";
import { Button, Popup, TreeView } from "../Components.ts";
import { useSession } from "../hooks/session.ts";
import { ApiError, ApiErrorType, DaDirectory, useApi } from "../hooks/api.ts";
import { TreeViewItemProps } from "../components/TreeView.tsx";

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

    const handleTreeViewNew = (name: string, parent?: TreeViewItemProps) => {
        return api?.postDirectory({
            name,
            parent: parent?.directory.sid,
        });
    };

    const handleTreeViewDelete = (props: TreeViewItemProps) => {
        return api?.deleteDirectory(props.directory.sid).then((_) => true);
    };

    const handleTreeViewRename = (
        props: TreeViewItemProps,
        newName: string,
    ) => {
        return api?.putDirectory(props.directory.sid, { name: newName });
    };

    useEffect(() => {
        if (api == null) {
            return;
        }
        api.getDirectory(0)
            .then((directory: DaDirectory | null) => {
                if (directory) {
                    setRootDirectory(directory);
                }
            })
            .catch((reason: ApiError) => {
                if (reason.kind == ApiErrorType.NotFound) {
                    console.log("No root directory, creating ...");
                    api.postDirectory({
                        name: "Root",
                    })
                        .then((directory: DaDirectory | null) => {
                            if (directory) {
                                setRootDirectory(directory);
                            }
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
            <div className="justify-start flex flex-row gap-4">
                <Button onClick={handleLogout}>Logout</Button>
            </div>
            <div className="flex flex-row grow pt-4 overflow-hidden">
                {rootDirectory != null
                    ? (
                        <TreeView
                            className="pt-4"
                            directory={rootDirectory}
                            onNew={handleTreeViewNew}
                            onDelete={handleTreeViewDelete}
                            onRename={handleTreeViewRename}
                        />
                    )
                    : <div>Loading ...</div>}
            </div>
            {currentError != null && (
                <Popup>
                    <div className="bg-slate-800 p-4 rounded-md">
                        <div className="text-red-500 text-xl">
                            An error occurred
                        </div>
                        <div className="pt-4 pb-4">
                            {currentError?.prettyMessage}
                        </div>
                        <Button onClick={() => setCurrentError(null)}>
                            Close
                        </Button>
                    </div>
                </Popup>
            )}
        </div>
    );
}

export default DirectoryView;
