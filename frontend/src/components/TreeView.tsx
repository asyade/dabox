import React, { useMemo, useState } from "react";
import { DaDirectory } from "../hooks/api.ts";
import { Button } from "../Components.ts";
import { ButtonKind } from "./Button.tsx";
import { useApi } from "../hooks/api.ts";

type TreeViewProps = {
    directory: DaDirectory;
    className?: string;
};

type TreeViewItemProps = {
    directory: DaDirectory;
    className?: string;
};

function TreeViewItem(props: TreeViewItemProps) {
    const api = useApi();
    const [children, setChildren] = useState<DaDirectory[]>(
        props.directory.children,
    );
    const [newDirectoryName, setNewDirectoryName] = useState<string | null>(
        null,
    );
    const handleInputBlur = (_) => {
        setNewDirectoryName(null);
    };

    const handleInputKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === "Enter") {
            setNewDirectoryName(null);
            api.postDirectory({
                name: newDirectoryName,
                parent: props.directory.sid,
            }).then((newDirectory) => {
                setChildren([...children, newDirectory]);
            });
        }
    };

    const handleNewDirectoryButtonClick = () => {
        setNewDirectoryName("New Directory");
    };

    const childrenViews = useMemo(() => {
        return children.map((child: DaDirectory) => (
            <TreeViewItem
                className="pl-4 border-l-2 border-gray-800"
                directory={child}
            />
        ));
    }, [children]);

    return (
        <div className={`flex flex-col w-full ${props.className}`}>
            <div className="flex flex-row items-center justify-between">
                <div className="p-1">
                    <span>{props.directory.name}</span>
                </div>
                {!newDirectoryName &&
                    (
                        <div className="w-1/8">
                            <Button
                                className="p-1"
                                kind={ButtonKind.Outline}
                                onClick={handleNewDirectoryButtonClick}
                            >
                                +
                            </Button>
                        </div>
                    )}
            </div>
            {newDirectoryName != null && (
                <div className="pl-4 w-full">
                    <input
                        value={newDirectoryName}
                        onChange={(e) => setNewDirectoryName(e.target.value)}
                        onBlur={handleInputBlur}
                        onKeyDown={handleInputKeyDown}
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
    return <TreeViewItem directory={props.directory} />;
}
