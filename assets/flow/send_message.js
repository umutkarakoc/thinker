let send_message = async (step) => {
	let el = render(step)
	el.addClass("send_message")

	el.find("header")
		.append(`<i class="fa-solid fa-paper-plane"></i>`)
		.append(`<span>Send Message</span>`);

	let textarea = $(`<textarea name="content" >${step.content}</textarea>`)
		.on("blur", async(e) => {
			await patch(`flow/step/send_message/${step.id}`, {content: e.target.value })
		});

	el.find("main")
		.append(textarea)
		.append(connector(step.id))
};
