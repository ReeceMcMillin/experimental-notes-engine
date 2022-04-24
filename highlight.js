// Copyright 2002-2010, Simon Marlow.  All rights reserved.
// https://github.com/haskell/haddock/blob/ghc-8.8/LICENSE
// Slightly modified by Tesla Ice Zhang
// Slightly modified (further) by Reece McMillin

let currentHover = null;

const references = document.getElementsByClassName('reference');

const highlight = (self, on) => () => {
    const definition = self.getAttribute("data-definition");
    if (definition) {
        if (currentHover) {
            currentHover.remove();
            currentHover = null;
        }

        if (on) {
            currentHover = document.createElement("div");
            currentHover.innerHTML = definition;
            currentHover.classList.add("definition");
            document.body.appendChild(currentHover);
            hljs.highlightAll();

            const selfRect = self.getBoundingClientRect();
            const hoverRect = currentHover.getBoundingClientRect();
            // If we're close to the bottom of the page, push the tooltip above instead.
            // The constant here is arbitrary, because trying to convert em to px in JS is a fool's errand.
            if (selfRect.bottom + hoverRect.height + 30 > window.innerHeight) {
                // 2em from the material mixin. I'm sorry
                currentHover.style.top = `${self.offsetTop - hoverRect.height - 20}px`;
                currentHover.style.left = `${self.offsetLeft - selfRect.width / 2 - hoverRect.width / 2}px`;
            } else {
                currentHover.style.top = `${self.offsetTop + self.offsetHeight}px`;
                currentHover.style.left = `${self.offsetLeft - hoverRect.width / 2 + selfRect.width / 2}px`;
            }
        }
    }

    Array.from(references).forEach(reference => {
        if (reference.getAttribute("data-definition") === definition) {
            reference.classList.toggle("highlight", on);
        }
    });
};

document.addEventListener('DOMContentLoaded', () => {
    Array.from(references).forEach(element => {
        if (element.hasAttribute('data-definition')) {
            element.onmouseover = highlight(element, true);
            element.onmouseout = highlight(element, false);
        }
    });
});