function updateValue(inputId, value) {
  $("#" + inputId).val(value);
}

async function activateAction(action_id, params = {}) {
  let url = '/api/v1/actions/' + action_id + '/activate/';
  const response = await fetch(url, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(params)
  });
  return true
}

function deleteAction(action_id) {
  let url = '/api/v1/actions/' + action_id;
  fetch(url, {
    method: 'DELETE',
  }).then(_ => location.reload())
}

$('.action-button').click(function (e) {
  e.preventDefault()
  let form = $(this).closest('form');
  let actionId = form.find('.actionId').val();
  let params = {};

  form.find('.paramName').each(function () {
    let name = $(this).val();
    let val = parseInt(form.find('#param-' + name).val());
    params[name] = val;
  });
  activateAction(actionId, params)
})