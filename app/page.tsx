"use client";

import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api";

import NavBar from "@/components/navBar";

export default function Home() {
  const [response, setResponse] = useState("");
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <NavBar />
      <div>The Response is {response}</div>
      <button
        onClick={() =>
          invoke<string>("login", { masterKey: "Next.js" }).then((res) => {
            setResponse(res);
          })
        }
      >
        Foobar
      </button>
    </main>
  );
}
