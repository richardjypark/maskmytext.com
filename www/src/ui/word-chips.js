export function createWordChip(word, onRemove) {
  const chip = document.createElement("div");
  chip.className = "word-chip";

  const text = document.createElement("span");
  text.textContent = word;

  const button = document.createElement("button");
  button.type = "button";
  button.title = "Remove word";
  button.textContent = "×";
  button.addEventListener("click", () => onRemove(word));

  chip.append(text, button);
  return chip;
}

export function renderWordChips(container, words, onRemove) {
  container.replaceChildren();

  for (const word of words) {
    container.appendChild(createWordChip(word, onRemove));
  }
}
