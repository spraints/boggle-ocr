const board = document.querySelector("#board")
const solveBtn = document.querySelector("#solve")
const solution = document.querySelector("#solution")

solveBtn.addEventListener("click", async function() {
  const resp = await fetch(`/boggle/solver/solution?board=${encodeURIComponent(board.value)}`)
  if (!resp.ok) {
    solution.innerText = await resp.text()
    return
  }
  const respData = await resp.json()

  const summary = document.createElement("h3")
  summary.innerText = `found ${respData.total_words} words, ${respData.total_score} points`

  const bestWordsTable = document.createElement("table")
  bestWordsTable.innerHTML = "<tr><th colspan=\"3\">Best words</th></tr><tr><th>Points</th><th>Word</th><th>Definition</th></tr>"

  for (bw of respData.best_words) {
    const tr = document.createElement("tr")
    const score = document.createElement("td")
    score.innerText = bw.score
    const word = document.createElement("td")
    word.innerText = bw.word
    const definition = document.createElement("td")
    definition.innerText = bw.def || ""
    tr.append(score, word, definition)
    bestWordsTable.append(tr)
  }

  solution.replaceChildren(summary, bestWordsTable)
})
