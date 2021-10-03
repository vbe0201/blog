(function () {
  const body = document.body;

  const setTheme = (theme) =>
    theme === "dark"
      ? body.setAttribute("dark", true)
      : body.removeAttribute("dark");

  const setLocalStorage = (theme) =>
    theme === "dark"
      ? localStorage.setItem("theme", "dark")
      : localStorage.removeItem("theme");

  setTheme(localStorage.getItem("theme"));

  const sun = document.getElementById("sun");
  sun.onclick = () => {
    if (localStorage.getItem("theme") === "dark") {
      setLocalStorage(null);
      setTheme(localStorage.getItem("theme"));
    } else {
      setLocalStorage("dark");
      setTheme(localStorage.getItem("theme"));
    }
  };

  const trigger = document.getElementById("popup-trigger");
  const navbar = document.getElementsByClassName("c-navbar__links")[0];
  const wrapper = document.getElementsByClassName("l-wrapper")[0];
  trigger.onclick = () => {
    console.log("Moin");
    if (!navbar.getAttribute("popup")) {
      navbar.setAttribute("popup", true);
      wrapper.classList.remove("blurry");
    } else {
      navbar.removeAttribute("popup");
      wrapper.classList.add("blurry");
    }
  };
})();
