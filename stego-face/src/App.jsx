import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
    const [greetMsg, setGreetMsg] = useState("");
    const [name, setName] = useState("");
    const [master_password, set_master_password] = useState("")
    const [data_name, set_data_name] = useState("")
    const [data, set_data] = useState("")
    const [file_path, set_file_path] = useState("")

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        setGreetMsg(await invoke("greet", { name }));
    }

    async function invoke_add_entry() {
        invoke('invoke_add_entry', { masterPassword: master_password, dataName: data_name, data: data, filePath: file_path })
    }

    return (
        <div className="container">
            <h1>Welcome to Tauri!</h1>

            <div className="row">
                <a href="https://vitejs.dev" target="_blank">
                    <img src="/vite.svg" className="logo vite" alt="Vite logo" />
                </a>
                <a href="https://tauri.app" target="_blank">
                    <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
                </a>
                <a href="https://reactjs.org" target="_blank">
                    <img src={reactLogo} className="logo react" alt="React logo" />
                </a>
            </div>

            <p>Click on the Tauri, Vite, and React logos to learn more.</p>

            <form
                className="row"
                onSubmit={(e) => {
                    e.preventDefault();
                    greet();
                }}
            >
                <input
                    id="greet-input"
                    onChange={(e) => setName(e.currentTarget.value)}
                    placeholder="Enter a name..."
                />
                <button type="submit">Greet</button>
            </form>

            <p>{greetMsg}</p>



            <p>stego</p>

            <form className="row"
                onSubmit={(e) => {
                    e.preventDefault();
                    invoke_add_entry()
                }}>

                <p> this form must only be shown to user if they are adding an entry (NOT READING)</p>
                <input
                    onChange={(e) => set_master_password(e.currentTarget.value)}
                    placeholder="Enter a master_password"
                />
                <input
                    onChange={(e) => set_data_name(e.currentTarget.value)}
                    placeholder="Enter data_name"
                />
                <input
                    onChange={(e) => set_data(e.currentTarget.value)}
                    placeholder="Enter data"
                />
                <input
                    onChange={(e) => set_file_path(e.currentTarget.value)}
                    placeholder="Enter a file_path"
                />
                <input type="submit" />
            </form>
        </div>
    );
}

export default App;
