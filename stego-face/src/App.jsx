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

  const [sidePanel, setSidePanel] = useState({ open: false, title: "", content: "" });

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
    setSidePanel({ open: false, title: "", content: "" });
    try {
      const stegoPath = await invoke("invoke_add_entry", {
        masterPassword: masterPasswordAdd,
        data: dataAdd,
        filePath: filePathAdd,
      });
      setFeedback({ type: "success", text: "Encoded." });
      try {
        const diff = await invoke("invoke_hex_diff", {
          originalPath: filePathAdd,
          stegoPath: stegoPath,
        });
        setSidePanel({ open: true, title: "hex diff", content: diff });
      } catch (_) {}
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
    setSidePanel({ open: false, title: "", content: "" });
    try {
      const data = await invoke("invoke_read_entry", {
        masterPassword: masterPasswordRead,
        filePath: filePathRead,
      });
      setSecretData(data);
      setFeedback({ type: "success", text: "Decoded." });
      try {
        const dump = await invoke("invoke_hex_dump", {
          filePath: filePathRead,
        });
        setSidePanel({ open: true, title: "hex dump", content: dump });
      } catch (_) {}
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
    <div className={`container${sidePanel.open ? " with-panel" : ""}`}>
      <div className="header">
        <h1>
          stego<span>suite</span>
        </h1>
        <span className="subtitle">hide messages in png images</span>
      </div>

      <div className="mode-toggle">
        <button
          className={mode === "encrypt" ? "active" : ""}
          onClick={() => { setMode("encrypt"); setFeedback(null); }}
        >
          encrypt
        </button>
        <button
          className={mode === "decrypt" ? "active" : ""}
          onClick={() => { setMode("decrypt"); setFeedback(null); }}
        >
          decrypt
        </button>
      </div>

      {mode === "encrypt" ? (
        <div className="card">
          <h2>new entry</h2>
          <form className="form-group" onSubmit={handleEncrypt}>
            <label>
              <span>password</span>
              <input
                type="password"
                placeholder="master password"
                value={masterPasswordAdd}
                onChange={(e) => setMasterPasswordAdd(e.target.value)}
              />
            </label>

            <label>
              <span>message</span>
              <input
                type="text"
                placeholder="text to hide"
                value={dataAdd}
                onChange={(e) => setDataAdd(e.target.value)}
              />
            </label>

            <label>
              <span>image</span>
              <button
                type="button"
                className={`file-picker${filePathAdd ? " has-file" : ""}`}
                onClick={() => pickFile(setFilePathAdd)}
              >
                <span className="file-icon">PNG</span>
                <span className="file-name">
                  {filePathAdd ? extractFileName(filePathAdd) : "select a png file"}
                </span>
              </button>
            </label>

            {encryptPreview && (
              <div className="image-preview">
                <img src={encryptPreview} alt="Preview" />
              </div>
            )}

            <div className="btn-wrapper">
              <button type="submit" className="btn-primary" disabled={loading}>
                {loading ? "encoding..." : "encode"}
              </button>
              {loading && <div className="progress-bar"><div className="progress-bar-fill" /></div>}
            </div>
          </form>

          {feedback && (
            <div className={`feedback ${feedback.type}`}>
              {feedback.type === "success" ? "+" : "!"} {feedback.text}
            </div>
          )}
        </div>
      ) : (
        <div className="card">
          <h2>read entry</h2>
          <form className="form-group" onSubmit={handleDecrypt}>
            <label>
              <span>password</span>
              <input
                type="password"
                placeholder="master password"
                value={masterPasswordRead}
                onChange={(e) => setMasterPasswordRead(e.target.value)}
              />
            </label>

            <label>
              <span>image</span>
              <button
                type="button"
                className={`file-picker${filePathRead ? " has-file" : ""}`}
                onClick={() => pickFile(setFilePathRead)}
              >
                <span className="file-icon">PNG</span>
                <span className="file-name">
                  {filePathRead ? extractFileName(filePathRead) : "select a png file"}
                </span>
              </button>
            </label>

            {decryptPreview && (
              <div className="image-preview">
                <img src={decryptPreview} alt="Preview" />
              </div>
            )}

            <div className="btn-wrapper">
              <button type="submit" className="btn-primary" disabled={loading}>
                {loading ? "decoding..." : "decode"}
              </button>
              {loading && <div className="progress-bar"><div className="progress-bar-fill" /></div>}
            </div>
          </form>

          {feedback && (
            <div className={`feedback ${feedback.type}`}>
              {feedback.type === "success" ? "+" : "!"} {feedback.text}
            </div>
          )}

          {secretData && (
            <div className="result">
              <div className="result-label">decoded</div>
              <div className="result-content">{secretData}</div>
              <button className="copy-btn" onClick={copyResult}>
                {copied ? "ok" : "copy"}
              </button>
            </div>
          )}
        </div>
      )}

      {sidePanel.open && (
        <div className="side-overlay" onClick={() => setSidePanel({ open: false, title: "", content: "" })}>
          <div className="side-panel" onClick={(e) => e.stopPropagation()}>
            <div className="side-panel-header">
              <span className="side-panel-title">{sidePanel.title}</span>
              <button className="side-panel-close" onClick={() => setSidePanel({ open: false, title: "", content: "" })}>x</button>
            </div>
            <div className="side-panel-content">
              <pre>{sidePanel.content}</pre>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
