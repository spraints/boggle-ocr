const word = document.querySelector("#word")
const result = document.querySelector("#result")

word.addEventListener("input", async function() {
  const s = word.value
  if (s == "") {
    result.innerText = ""
    return
  }
  const resp = await fetch(`/boggle/dict/words/${encodeURIComponent(s)}`)
  const res = await resp.text()
  if (s == word.value) {
    if (resp.status == 200) {
      result.innerText = `definition of ${s}: ${res}`
    } else if (resp.status == 404) {
      result.innerText = `${s}: not a word`
    } else {
      result.innerText = `HTTP ${resp.status} ${res}`
    }
  }
})
