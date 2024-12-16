export async function getAsset(name) {
    const res = await fetch(`./${name}`);
    const blob = await res.blob();
    const text = await blob.text();
    return text;
}