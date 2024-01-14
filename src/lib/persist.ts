const PREFIX = "hoyo-game-starter";

export function getStorage(key: keyof typeof GameKey): Hoyo {
  const item = window.localStorage.getItem(`${PREFIX}-${key}`);
  let hoyo: Hoyo = {
    root: "",
    launcher: "",
    game: "",
    exe: "",
  };
  if (item != null) {
    hoyo = JSON.parse(item);
  }
  return hoyo;
}

export function setStorage(key: keyof typeof GameKey, hoyo: Hoyo): void {
  window.localStorage.setItem(`${PREFIX}-${key}`, JSON.stringify(hoyo));
}
