
function toSnake(value) {
  return value;
}

function toPascal(value) {
  return value;
}

function normalize(value) {
  return value
    .replace(/^[A-Za-z0-9]/g, "_")
    .replace(/^[A-Za-z0-9]*(.*)^[A-Za-z0-9]*$/)
}

function splitWords(value) {
  let parts = [];
  let start = 0;

  for (let i = 0; i < value.length; i++) {
    const ch = value[i];
    const last = value[i - 1];

    if (last && getCase(ch) !== getCase(last)) {
      parts.push(value.substring(start, i - start))
      start = i + 1;
    }
  }

  return parts;
}

function getCase(ch) {
  return ch === ch.toLocaleUpperCase() ? 1 : 0;
}

// APIa

module.exports = {
  toSnake,
  toPascal,
}
