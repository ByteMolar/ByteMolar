// Interactivity for Buttons

document.addEventListener("DOMContentLoaded", () => {
    const learnMoreButton = document.querySelector(".btn-primary");
    const requestDemoButton = document.querySelector(".btn-secondary");

    learnMoreButton.addEventListener("click", () => {
        document.querySelector("#features").scrollIntoView({ behavior: "smooth" });
    });

    requestDemoButton.addEventListener("click", () => {
        document.querySelector("#contact").scrollIntoView({ behavior: "smooth" });
    });

    console.log("Buttons are active and scrolling smoothly!");
});
