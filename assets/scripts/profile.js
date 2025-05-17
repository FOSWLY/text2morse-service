const morseInputEl = document.getElementById("morse-input");
const morseOutputEl = document.getElementById("morse-output");
const profileCont = document.querySelector(".profile-body");

const directions = [
  { label: "Text ➔ Morse", value: "text" },
  { label: "Morse ➔ Text", value: "morse" },
];
const alphabets = [
  { label: "ASCII", value: 1 },
  { label: "Numbers", value: 2 },
  { label: "Punctuation", value: 3 },
  { label: "Latin Extended", value: 4 },
  { label: "Cyrillic", value: 5 },
  { label: "Greek", value: 6 },
  { label: "Hebrew", value: 7 },
  { label: "Arabic", value: 8 },
  { label: "Persian", value: 9 },
  { label: "Japanese", value: 10 },
  { label: "Korean", value: 11 },
  { label: "Thai", value: 12 },
];

let selectedAlphabet = alphabets[0];
let selectedDirection = directions[0];
let morseFn = window.morse.encode;

const callMorse = (text) => {
  return morseFn(text, {
    priority: selectedAlphabet.value,
  });
};

morseInputEl.addEventListener("input", (event) => {
  morseOutputEl.textContent = callMorse(event.target.value);
});

const directionDropdown = createDropdown({
  title: "Direction",
  selected: selectedDirection,
  search: false,
  options: directions,
  onSelect: (option) => {
    morseFn =
      option.value === "text" ? window.morse.encode : window.morse.decode;
    morseInputEl.value = morseOutputEl.textContent.trim();
    morseOutputEl.textContent = callMorse(morseInputEl.value);
  },
});

const alphabetDropdown = createDropdown({
  title: "Alphabet",
  selected: selectedAlphabet,
  search: true,
  options: alphabets,
  onSelect: (option) => {
    selectedAlphabet = option;
    morseOutputEl.textContent = callMorse(morseInputEl.value);
  },
});

profileCont.prepend(
  directionDropdown.containerEl,
  alphabetDropdown.containerEl
);
