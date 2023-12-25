htmx.on("htmx:afterRequest", function(evt) {
    let chatArea = document.querySelector('.chat-area');
    let input = document.querySelector('.input');

    if (!chatArea || !input) return;

    chatArea.childNodes[chatArea.childNodes.length - 1].scrollIntoView(true);

    if (evt.detail.pathInfo.requestPath === "/show-my-message") {
        input.value = "";
    }
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
