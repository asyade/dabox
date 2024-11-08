import React, { useEffect, useMemo, useState } from "react";
import { ApiError, DaDirectory, useApi } from "../hooks/api.ts";
import { Button } from "../Components.ts";
import { ButtonKind } from "./Button.tsx";

export type TreeViewProps = {
    directory?: DaDirectory;
    className?: string;
    onDelete?: (item: TreeViewItemProps) => Promise<boolean>;
    onNew?: (
        name: string,
        parent?: TreeViewItemProps,
    ) => Promise<DaDirectory>;
    onRename?: (item: TreeViewItemProps, newName: string) => Promise<boolean>;
};

export type TreeViewItemProps = {
    /// The parent item
    parent?: TreeViewItemProps;
    /// Called when the item (self) is deleted
    onDeleted?: (item: TreeViewItemProps) => void;
    /// The directory this item represents  (came from the API)
    directory: DaDirectory;
    /// Additional CSS class name
    className?: string;
    /// Root props
    treeViewProps: TreeViewProps;
};

function TreeViewItem(props: TreeViewItemProps) {
    const [children, setChildren] = useState<DaDirectory[]>(
        props.directory.children,
    );
    const [newDirectoryName, setNewDirectoryName] = useState<string | null>(
        null,
    );

    const [renamedDirectoryName, setRenamedDirectoryName] = useState<
        string | null
    >(
        null,
    );

    const handleNewDirectoryInputKeyDown = (
        e: React.KeyboardEvent<HTMLInputElement>,
    ) => {
        if (e.key === "Enter") {
            if (!newDirectoryName || newDirectoryName == "") {
                return;
            }
            props.treeViewProps.onNew?.(newDirectoryName, props)
                .then((newDirectory: DaDirectory | null) => {
                    if (newDirectory) {
                        setChildren([...children, newDirectory]);
                    }
                });
            setNewDirectoryName(null);
        }
    };

    const handleDeleteDirectory = () => {
        props.treeViewProps.onDelete?.(props)
            .then((success: boolean) => {
                if (success) {
                    props.onDeleted?.(props);
                }
            });
    };

    const handleRenameDirectory = () => {
        if (renamedDirectoryName == null) {
            console.error("renamedDirectoryName is null");
            return;
        }
        props.treeViewProps.onRename?.(props, renamedDirectoryName)
            .then((success: boolean) => {
                if (success) {
                    props.directory.name = renamedDirectoryName;
                }
                setRenamedDirectoryName(null);
            })
            .catch((reason: ApiError) => {
                console.error(reason);
                setRenamedDirectoryName(null);
            });
    };

    const childrenViews = useMemo(() => {
        return children.map((child: DaDirectory) => (
            <TreeViewItem
                onDeleted={() => {
                    setChildren(children.filter((c) => c.sid !== child.sid));
                }}
                treeViewProps={props.treeViewProps}
                parent={props}
                key={child.sid}
                className="pl-4 border-l-2 border-gray-800"
                directory={child}
            />
        ));
    }, [children]);

    return (
        <div className={`flex flex-col w-full ${props.className}`}>
            <div className="flex flex-row items-center justify-between">
                <div className="p-1">
                    {renamedDirectoryName == null
                        ? (
                            <span
                                onClick={() =>
                                    setRenamedDirectoryName(
                                        props.directory.name,
                                    )}
                            >
                                {props.directory.name}
                            </span>
                        )
                        : (
                            <input
                                className="text-white w-full bg-transparent placeholder:text-white-400 text-white-700 text-sm border border-white-200 rounded-md px-3 py-2 transition duration-300 ease focus:outline-none focus:border-teal-500 hover:border-teal-300 shadow-sm focus:shadow"
                                value={renamedDirectoryName}
                                onChange={(e) =>
                                    setRenamedDirectoryName(e.target.value)}
                                onBlur={() => handleRenameDirectory()}
                            />
                        )}
                </div>
                <div className="flex flex-row">
                    {
                        // *Note*: We dont want to show the delete button for the root directory, it's actually can be deleted but
                        // Doing so will require a session system to store the root directory id (as it's will not be 0 anymore)
                        props.parent != null && (
                            <div className="w-1/8">
                                <Button
                                    className="p-1"
                                    kind={ButtonKind.Outline}
                                    onClick={() => handleDeleteDirectory()}
                                >
                                    -
                                </Button>
                            </div>
                        )
                    }
                    <div className="w-1/8">
                        <Button
                            className="p-1"
                            kind={ButtonKind.Outline}
                            onClick={() => setNewDirectoryName("")}
                        >
                            +
                        </Button>
                    </div>
                </div>
            </div>
            {newDirectoryName != null && (
                <div className="pl-6 w-full">
                    <input
                        value={newDirectoryName}
                        onChange={(e) => setNewDirectoryName(e.target.value)}
                        onBlur={() => setNewDirectoryName(null)}
                        onKeyDown={handleNewDirectoryInputKeyDown}
                        type="text"
                        className="text-white w-full bg-transparent placeholder:text-white-400 text-white-700 text-sm border border-white-200 rounded-md px-3 py-2 transition duration-300 ease focus:outline-none focus:border-teal-500 hover:border-teal-300 shadow-sm focus:shadow"
                        autoFocus={true}
                    />
                </div>
            )}
            <div className="pl-4">{childrenViews}</div>
        </div>
    );
}

export function TreeView(props: TreeViewProps) {
    if (props.directory == null) {
        return <></>;
    }
    return (
        <TreeViewItem
            onDeleted={() => {}}
            treeViewProps={props}
            directory={props.directory}
        />
    );
}
