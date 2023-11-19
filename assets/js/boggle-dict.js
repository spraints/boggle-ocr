const word = document.querySelector("#word")
const results = document.querySelectorAll(".result")
const resultFound = document.querySelector(".dictionary-entry")
const foundWord = document.querySelector(".dictionary-word")
const foundDefinition = document.querySelector(".dictionary-definition")
const resultError = document.querySelector(".not-found")

word.addEventListener("input", async function() {
  const s = word.value
  for (e of results) {
    e.classList.add("result-hidden")
  }
  if (s == "") {
    return
  }

  try {
    const resp = await fetch(`/boggle/dict/words/${encodeURIComponent(s)}`)
    const res = await resp.text()
    if (s == word.value) {
      for (e of results) {
        e.classList.add("result-hidden")
      }
      if (resp.status == 200) {
        foundWord.innerText = s
        foundDefinition.innerText = res
        resultFound.classList.remove("result-hidden")
      } else if (resp.status == 404) {
        resultError.innerText = `${s}: not a word`
        resultError.classList.remove("result-hidden")
      } else {
        resultError.innerText = `HTTP ${resp.status} ${res}`
        resultError.classList.remove("result-hidden")
      }
    }
  } catch {
    if (s == word.value) {
      for (e of results) {
        e.classList.add("result-hidden")
      }
      resultError.innerText = "server unavailable"
      resultError.classList.remove("result-hidden")
    }
  }
})
