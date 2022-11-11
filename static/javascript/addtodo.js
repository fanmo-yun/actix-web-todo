async function delete_todo(title, time) {
    await fetch('/api/app/' + title + '/' + time, {
        method: 'delete'
    });

    location.reload();
}