import { writable } from 'svelte/store';

export const pairs = writable([]);

export function addPair(a, b) {
    const newPair = [a, b].sort((x, y) => x - y);

    pairs.update((currentArr) => {
        if (currentArr.some(pair => pair.every((val, idx) => val === newPair[idx]))) {
            return currentArr;
        }
        return [...currentArr, newPair];
    });
}

export function removePair(index) {
    pairs.update((currentArr) => {
        if (index >= 0 && index < currentArr.length) {
            return currentArr.filter((_, i) => i !== index);
        }
        return currentArr;
    });
}

