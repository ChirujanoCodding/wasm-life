import { Universe } from 'game-of-life';

const FRAMES = 15;
const pre = document.getElementById("game-of-life");
const universe = Universe.new();

const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();

    setTimeout(() => {
        requestAnimationFrame(renderLoop);
    }, 1000 / FRAMES)
};

renderLoop()