import "./App.css";

import React from "react";
import { CookiesProvider } from "react-cookie";
import { useSession } from "./hooks/session.ts";
import DirectoryView from "./views/DirectoryView.tsx";
import LoginView from "./views/LoginView.tsx";

function App() {
  const [session] = useSession();
  console.log(session);
  return (
    <CookiesProvider>
      <div
        style={{ width: "100vw", height: "100vh" }}
        className="flex min-h-full min-w-screen flex-col justify-center px-6 py-12 lg:px-8"
      >
        {session != null ? <DirectoryView /> : <LoginView />}
      </div>
    </CookiesProvider>
  );
}

export default App;
