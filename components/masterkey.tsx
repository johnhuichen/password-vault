"use client";

import { ChangeEvent, FormEvent, useState } from "react";

import { invoke } from "@tauri-apps/api/tauri";

export default function MasterKey() {
  const [masterKey, setMasterKey] = useState("");
  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    setMasterKey(e.target.value);
  };

  const handleSubmit = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    invoke<string>("encrypt", { keyStr: masterKey, plaintext: "encryption" })
      .then((result) => console.log(result))
      .catch(console.error);
    console.log(masterKey);
  };

  return (
    <div className="p-20 m-10 border-2 border-solid border-black">
      <form onSubmit={handleSubmit}>
        <label>Masterkey:</label>
        <input type="password" onChange={handleChange} />
        <input type="submit" value="submit" />
      </form>
    </div>
  );
}
