import React, { useState } from "react";
import { useSession } from "../hooks/session.ts";
import { Button } from "../Components.ts";
function LoginView() {
    const [userIdInput, setUserIdInput] = useState<number | null>(null);
    const [_, setUserId] = useSession();

    const handleUserIdChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setUserIdInput(parseInt((e.target as any).value));
    };

    const handleLogin = () => {
        if (userIdInput != null) {
            setUserId(userIdInput.toString());
        }
    };

    return (
        <div className="flex min-h-full flex-col justify-center">
            <div className="sm:mx-auto sm:w-full sm:max-w-sm">
                <div className="sm:mx-auto sm:w-full sm:max-w-sm">
                    <span className="text-md/6 font-bold">
                        Choose a user id bellow or any non-existing user to
                        start from an empty directory
                    </span>
                    <div className="text-sm/6">
                        <br />
                        <span>42 (500k directories)</span>
                        <br />
                        <span>0 (1k directories)</span>
                    </div>
                </div>
                <div className="mt-10 space-y-6">
                    <div>
                        <label
                            htmlFor="user-id"
                            className="block text-sm/6 font-medium"
                        >
                            User id
                        </label>
                        <div className="mt-2">
                            <input
                                value={userIdInput ?? ""}
                                onChange={handleUserIdChange}
                                id="user-id"
                                name="user-id"
                                type="number"
                                required
                                className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm/6"
                            />
                        </div>
                    </div>
                    <div className="mt-10">
                        <Button onClick={handleLogin}>Login</Button>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default LoginView;
