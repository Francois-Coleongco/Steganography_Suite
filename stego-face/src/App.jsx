import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-shell";
import "./App.css";

function App() {
    const [master_password_add_entry, set_master_password_add_entry] = useState("")
    const [data_add, set_data_add] = useState("")
    const [file_path_add, set_file_path_add] = useState("")

    const [master_password_read_entry, set_master_password_read_entry] = useState("")
    const [file_path_read, set_file_path_read] = useState("")

    const [secwet_data, set_secwet_data] = useState("")


    const [toggleAddRead, setToggleAddRead] = useState(0)


    async function invoke_add_entry() {
        console.log(file_path_add)
        invoke('invoke_add_entry', { masterPassword: master_password_add_entry, data: data_add, filePath: file_path_add })
    }

    async function invoke_read_entry() {
        invoke('invoke_read_entry', { masterPassword: master_password_read_entry, filePath: file_path_read }).then((decrypted_data) => {
            set_secwet_data(decrypted_data)
        })
    }

    async function invoke_read_entry_handler() {
        await invoke_read_entry()
        console.log(secwet_data)
    }


    useEffect(() => {
        console.log(file_path_add)
        console.log(file_path_read)
    })

    if (toggleAddRead === 0) {
        return (
            <div className="container">

                <h1>Encrypted Steganography Suite</h1>

                <br />
                <button onClick={() => { setToggleAddRead(1) }}>switch to decrypt</button>
                <form
                    id="add_entry"
                    onSubmit={(e) => {
                        e.preventDefault();
                        invoke_add_entry()
                    }}>

                    <br />
                    <input
                        onChange={(e) => set_master_password_add_entry(e.currentTarget.value)}
                        placeholder="Enter a master_password_add_entry"
                    />
                    <br />
                    <input
                        onChange={(e) => set_data_add(e.currentTarget.value)}
                        placeholder="Enter data"
                    />
                    <br />
                    <input
                        onChange={(e) => set_file_path_add(e.currentTarget.value)}
                        placeholder="Select a file_path"
                    />
                    <br />
                    <input type="submit" />
                </form>
            </div>
        );
    }

    else {

        return (
            <div className="container">

                <h1>Encrypted Steganography Suite</h1>
                <br />
                <button onClick={() => { setToggleAddRead(0) }}>switch to encrypt</button>
                <p>read entry</p>

                <form
                    id="read_entry"
                    onSubmit={(e) => {
                        e.preventDefault()
                        invoke_read_entry_handler()
                    }}

                >
                    <input
                        onChange={(e) => set_master_password_read_entry(e.currentTarget.value)}
                        placeholder="Enter a master_password"
                    />
                    <br />
                    <input
                        onChange={(e) => set_file_path_read(e.currentTarget.value)}
                        placeholder="Select a file_path"
                    />
                    <br />
                    <input type="submit" />
                </form>



                <p> {secwet_data}</p>
            </div>
        )
    }

}
export default App;
