htmx.on("htmx:afterRequest", function(evt) {
    let chatArea = document.querySelector('.chat-area');
    let input = document.querySelector('.input');

    if (!chatArea || !input) return;

    if (evt.detail.pathInfo.requestPath === "/show-my-message") {
        input.value = "";
    }

    let lastMessage = Array.from(chatArea.childNodes)
        .filter(n => n.classList && Array.from(n.classList).find(c => c === "chat-message"))
        .slice(-1)
        .pop();
    lastMessage?.scrollIntoView(true);
});

htmx.on("htmx:confirm", function(evt) {
    let input = document.querySelector('.input');

    if (!input.value || !input.value?.trim()) evt.preventDefault();
});

htmx.on("htmx:afterSwap", function(evt) {
    let sendButton = document.querySelector('.send-button');
    sendButton.disabled = true;

    htmx.ajax("POST", "/get-bot-response", {
        target: ".chat-area",
        swap: "beforeend",
        values: evt.detail.requestConfig.parameters,
    }).then(() => sendButton.disabled = false);
});

function handleKeyup(e) {
    let sendButton = document.querySelector('.send-button');

    if (e.key === 'Enter' && !sendButton.disabled) {
        sendButton.click();
    }
};
