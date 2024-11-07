import React from "react";

export enum ButtonKind {
    Primary,
    Outline,
}

export function Button(props: {
    onClick: () => void;
    children: React.ReactNode;
    kind?: ButtonKind;
    className?: string;
}) {
    if (props.kind === ButtonKind.Outline) {
        return (
            <button
                onClick={() => props.onClick()}
                className={`flex w-full justify-center rounded-md bg-transparent text-indigo-500 hover:text-white hover:bg-indigo-500 ${props.className}`}
            >
                {props.children}
            </button>
        );
    } else {
        return (
            <button
                onClick={() => props.onClick()}
                className={`flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm/6 font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 ${props.className}`}
            >
                {props.children}
            </button>
        );
    }
}
