(function get_votes() {
    let names = [...document.querySelectorAll(".styles_titleTaglineItem__d5Rut")].map(item => item.textContent)
    let votes = [...document.querySelectorAll(".styles_voteCountItem__zwuqk")].map(item => parseInt(item.textContent))

    return names.map((name, i) => ({
        name,
        votes: votes[i]
    })).filter(item => !isNaN(item.votes)).sort((a, b) => b.votes - a.votes)
})()