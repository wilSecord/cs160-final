let num_connected_maxs = num_connected.reduce((acc, x) => {
    for (const key in x) {
        if (!acc[key] || x[key] >= acc[key]) acc[key] = x[key]
    }
    return acc;
}, {});

let num_connected_mins = num_connected.reduce((acc, x) => {
    for (const key in x) {
        if (!acc[key] || x[key] <= acc[key]) acc[key] = x[key]
    }
    return acc;
}, {});

let num_connected_norm = num_connected.map(x =>
    Object.fromEntries(Object.entries(x).map(([k, v]) => [k,
        k == "name" ? v : (v - num_connected_mins[k]) / (num_connected_maxs[k] - num_connected_mins[k])
    ]))
)

for (const state of num_connected_norm) {

    let hex = document.querySelector(`[data-state-name="${state.name}"]`)
    if (!hex) continue;
    hex.style.fill = `hsl(60deg, 60%, ${state.max * 30 + 70}%)`
}