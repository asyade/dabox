import React from "react";

export default function Button(props: {
    onClick: () => void;
    children: React.ReactNode;
}) {
    return (
        <button
            onClick={() => props.onClick()}
            className="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm/6 font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
        >
            {props.children}
        </button>
    );
}
