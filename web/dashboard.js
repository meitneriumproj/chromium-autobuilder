const logs = document.getElementById("logs")
const downloadBtn = document.getElementById("downloadBtn")

function detectOS() {

```
const ua = navigator.userAgent

if (ua.includes("Windows"))
    return "windows"

if (ua.includes("Mac"))
    return "macos"

if (ua.includes("Linux"))
    return "linux"

return "unknown"
```

}

function downloadLink(os) {

```
const base =
"https://github.com/YOURORG/YOURREPO/releases/latest/download/"

if (os === "windows")
    return base + "chromium-win.exe"

if (os === "macos")
    return base + "chromium-macos.dmg"

if (os === "linux")
    return base + "chromium-linux.tar.xz"

return "#"
```

}

function downloadLabel(os) {

```
if (os === "windows")
    return "Download for Windows"

if (os === "macos")
    return "Download for macOS"

if (os === "linux")
    return "Download for Linux"

return "Download Chromium"
```

}

async function loadBuildInfo() {

```
const res = await fetch("artifacts/latest/build.json")
const data = await res.json()

document.getElementById("version").textContent = data.version
document.getElementById("commit").textContent = data.commit
document.getElementById("time").textContent = data.commit_time
```

}

function setupDownload() {

```
const os = detectOS()

const url = downloadLink(os)

downloadBtn.href = url
downloadBtn.textContent = downloadLabel(os)
```

}

function connectLogs() {

```
const ws = new WebSocket("ws://" + location.host + "/ws")

ws.onmessage = (event) => {

    logs.textContent += event.data + "\n"

    logs.scrollTop = logs.scrollHeight
}
```

}

setupDownload()
loadBuildInfo()
connectLogs()
