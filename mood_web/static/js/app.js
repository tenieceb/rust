document.addEventListener("DOMContentLoaded", () => {
  const otherRadio = document.getElementById("emotion-other");
  const otherInput = document.getElementById("other-input");
  const radios = document.querySelectorAll('input[name="emotion"]');

  function syncOther() {
    const isOther = otherRadio.checked;
    otherInput.disabled = !isOther;
    otherInput.required = isOther;
    if (!isOther) otherInput.value = "";
  }

  radios.forEach(r => r.addEventListener("change", syncOther));
  syncOther();
});
