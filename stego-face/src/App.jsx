import { useState } from "react";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { open as openFileDialog } from "@tauri-apps/plugin-dialog";
import "./App.css";

function App() {
  const [masterPasswordAdd, setMasterPasswordAdd] = useState("");
  const [dataAdd, setDataAdd] = useState("");
  const [filePathAdd, setFilePathAdd] = useState("");

  const [masterPasswordRead, setMasterPasswordRead] = useState("");
  const [filePathRead, setFilePathRead] = useState("");

  const [secretData, setSecretData] = useState("");
  const [mode, setMode] = useState("encrypt");
  const [loading, setLoading] = useState(false);
  const [feedback, setFeedback] = useState(null);
  const [copied, setCopied] = useState(false);

  function extractFileName(path) {
    if (!path) return "";
    return path.split(/[/\\]/).pop();
  }

  function imagePreviewSrc(path) {
    if (!path) return null;
    return convertFileSrc(path);
  }

  async function pickFile(setter) {
    const selected = await openFileDialog({
      multiple: false,
      filters: [{ name: "PNG", extensions: ["png"] }],
    });
    if (selected) {
      setter(selected);
      setFeedback(null);
    }
  }

  async function handleEncrypt(e) {
    e.preventDefault();
    setLoading(true);
    setFeedback(null);
    try {
      await invoke("invoke_add_entry", {
        masterPassword: masterPasswordAdd,
        data: dataAdd,
        filePath: filePathAdd,
      });
      setFeedback({ type: "success", text: "Message encoded successfully." });
    } catch (err) {
      setFeedback({ type: "error", text: String(err) });
    } finally {
      setLoading(false);
    }
  }

  async function handleDecrypt(e) {
    e.preventDefault();
    setLoading(true);
    setFeedback(null);
    setSecretData("");
    try {
      const data = await invoke("invoke_read_entry", {
        masterPassword: masterPasswordRead,
        filePath: filePathRead,
      });
      setSecretData(data);
      setFeedback({ type: "success", text: "Message decoded successfully." });
    } catch (err) {
      setFeedback({ type: "error", text: String(err) });
    } finally {
      setLoading(false);
    }
  }

  async function copyResult() {
    try {
      await navigator.clipboard.writeText(secretData);
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    } catch {
      const ta = document.createElement("textarea");
      ta.value = secretData;
      document.body.appendChild(ta);
      ta.select();
      document.execCommand("copy");
      document.body.removeChild(ta);
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    }
  }

  const encryptPreview = imagePreviewSrc(filePathAdd);
  const decryptPreview = imagePreviewSrc(filePathRead);

  return (
    <div className="container">
      <div className="header">
        <h1>Steganography Suite</h1>
        <span className="subtitle">Encode and decode hidden messages in PNG images</span>
      </div>

      <div className="mode-toggle">
        <button
          className={mode === "encrypt" ? "active" : ""}
          onClick={() => { setMode("encrypt"); setFeedback(null); }}
        >
          Encrypt
        </button>
        <button
          className={mode === "decrypt" ? "active" : ""}
          onClick={() => { setMode("decrypt"); setFeedback(null); }}
        >
          Decrypt
        </button>
      </div>

      {mode === "encrypt" ? (
        <div className="card">
          <h2>Encrypt a message</h2>
          <form className="form-group" onSubmit={handleEncrypt}>
            <label>
              <span>Master Password</span>
              <input
                type="password"
                placeholder="Enter a master password"
                value={masterPasswordAdd}
                onChange={(e) => setMasterPasswordAdd(e.target.value)}
              />
            </label>

            <label>
              <span>Secret Message</span>
              <input
                type="text"
                placeholder="Enter the message to hide"
                value={dataAdd}
                onChange={(e) => setDataAdd(e.target.value)}
              />
            </label>

            <label>
              <span>PNG Image</span>
              <button
                type="button"
                className={`file-picker${filePathAdd ? " has-file" : ""}`}
                onClick={() => pickFile(setFilePathAdd)}
              >
                <span className="file-icon">📷</span>
                <span className="file-name">
                  {filePathAdd ? extractFileName(filePathAdd) : "Select a PNG image"}
                </span>
              </button>
            </label>

            {encryptPreview && (
              <div className="image-preview">
                <img src={encryptPreview} alt="Selected PNG preview" />
              </div>
            )}

            <button type="submit" className="btn-primary" disabled={loading}>
              {loading ? <><span className="spinner" /> Encoding...</> : "Encode Message"}
            </button>
          </form>

          {feedback && (
            <div className={`feedback ${feedback.type}`}>
              {feedback.type === "success" ? "\u2713" : "\u2715"} {feedback.text}
            </div>
          )}
        </div>
      ) : (
        <div className="card">
          <h2>Decrypt a message</h2>
          <form className="form-group" onSubmit={handleDecrypt}>
            <label>
              <span>Master Password</span>
              <input
                type="password"
                placeholder="Enter the master password"
                value={masterPasswordRead}
                onChange={(e) => setMasterPasswordRead(e.target.value)}
              />
            </label>

            <label>
              <span>PNG Image</span>
              <button
                type="button"
                className={`file-picker${filePathRead ? " has-file" : ""}`}
                onClick={() => pickFile(setFilePathRead)}
              >
                <span className="file-icon">📷</span>
                <span className="file-name">
                  {filePathRead ? extractFileName(filePathRead) : "Select a PNG image"}
                </span>
              </button>
            </label>

            {decryptPreview && (
              <div className="image-preview">
                <img src={decryptPreview} alt="Selected PNG preview" />
              </div>
            )}

            <button type="submit" className="btn-primary" disabled={loading}>
              {loading ? <><span className="spinner" /> Decoding...</> : "Decode Message"}
            </button>
          </form>

          {feedback && (
            <div className={`feedback ${feedback.type}`}>
              {feedback.type === "success" ? "\u2713" : "\u2715"} {feedback.text}
            </div>
          )}

          {secretData && (
            <div className="result">
              <div className="result-label">Decoded message</div>
              <div className="result-content">{secretData}</div>
              <button className="copy-btn" onClick={copyResult}>
                {copied ? "Copied!" : "Copy"}
              </button>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

export default App;