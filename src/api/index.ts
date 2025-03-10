import { fetch } from "@tauri-apps/api/http";

const staticResourceUrl = {
  bh3: "https://hyp-api.mihoyo.com/hyp/hyp-connect/api/getGamePackages?launcher_id=jGHBHlcOq1&game_ids[]=osvnlOc0S8",
  ys: "https://hyp-api.mihoyo.com/hyp/hyp-connect/api/getGamePackages?launcher_id=jGHBHlcOq1&game_ids[]=1Z8W5NHUQb",
  star: "https://hyp-api.mihoyo.com/hyp/hyp-connect/api/getGamePackages?launcher_id=jGHBHlcOq1&game_ids[]=64kMb5iAWu",
  zzz: "https://hyp-api.mihoyo.com/hyp/hyp-connect/api/getGamePackages?launcher_id=jGHBHlcOq1&game_ids[]=x6znKlJ0xK",
};

interface CheckRes {
  remoteVersion: string;
  hasPreDownload: boolean;
}

export const checkResource = async (key: GameKey) => {
  const url = staticResourceUrl[key];
  const checkRes: CheckRes = {
    remoteVersion: "",
    hasPreDownload: false,
  };
  try {
    const { data } = await fetch<StaticResource>(url);
    checkRes.remoteVersion = data.data.game_packages?.[0].main.major.version;
    checkRes.hasPreDownload =
      data.data.game_packages?.[0].pre_download?.major != null;
  } catch (error) {
    console.log(error);
  }
  console.log(checkRes);
  return checkRes;
};
