let render_branch = (branch, step, step_el) => {

		let branch_el = $(`<article class="branch">
				<main>
        <label for="text_${branch.id}">
          <input id="text_${branch.id}" type="text" name="text" value="${branch.text}" />
        </label>

        <label for="contains_${branch.id}">
          <input type="checkbox" id="contains_${branch.id}" name="contains" ${branch.contains ? "checked" : ""}>
          Contains
        </label>

        <label for="fuzzy_${branch.id}">
          <input type="checkbox" id="fuzzy_${branch.id}" name="fuzzy"${branch.fuzzy ? "checked" : ""}>
          Fuzzy match
        </label>
        </main>
      </article>`);

		branch_el.find("input").on("blur", async(e) => {
			await patch(`flow/step/update_wait_for_reply_branch`, {
				id: branch.id,
		    text: $(`#text_${branch.id}`).val(),
		    contains: $(`#contains_${branch.id}`).prop("checked"),
		    fuzzy: $(`#fuzzy_${branch.id}`).prop("checked")
			})
		});

		branch_el.appendTo(step_el.find(".branch_list"));
		branch_el.find("main").append(connector(branch.id));
}

let add_ask_question_branch = async (ev, step, step_el) => {
	let branch = await api("flow/step/add_wait_for_reply_branch", {ask_question_id: step.id});

	step.branches.push(branch);
	render_branch(branch, step, step_el)
}

let wait_for_reply = async (step) => {
	let el = render(step)
	el.addClass("wait_for_reply")

	el.find("header")
		.append(`<i class="fa-solid fa-code-fork"></i>`)
		.append(`<span>Wait For Reply</span>`);

	el.find("main")
		.append(`<div class="branch_list" id="branches_${step.id}"></div>`);

	step.branches = await api("flow/step/wait_for_reply_branch_list", { step_id: step.id });

	step.branches.forEach((branch) => render_branch(branch, step, el));

	$(`<button class="action small">Add new reply</button>`)
		.on("click", (ev) => add_ask_question_branch(ev, step, el))
		.appendTo(el.children("header"))

}