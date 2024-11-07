import React, { useEffect, useState } from "react";
import { useCookies } from "react-cookie";

type Cookies = {
    userId: string;
};

export function useSession() {
    const [cookies, setCookie] = useCookies(["user_id"]);

    const [userId, setUserId] = useState<string | undefined | null>(
        (cookies as Cookies).userId,
    );

    const setUserIdExternal = (userId: string) => {
        setCookie("user_id", userId);
    };

    useEffect(() => {
        if (cookies["user_id"] != userId) {
            setUserId(cookies["user_id"]);
        }
    }, [cookies]);

    return [userId, setUserIdExternal] as const;
}
