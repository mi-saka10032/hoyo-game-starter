import { fetch } from "@tauri-apps/api/http";

const staticResourceUrl = {
  bh3: "https://bh3-launcher-static.mihoyo.com/bh3_cn/mdk/launcher/api/resource?channel_id=1&key=SyvuPnqL&launcher_id=4&sub_channel_id=1",
  ys: "https://sdk-static.mihoyo.com/hk4e_cn/mdk/launcher/api/resource?channel_id=1&key=eYd89JmJ&launcher_id=18&sub_channel_id=1",
  star: "https://api-launcher-static.mihoyo.com/hkrpg_cn/mdk/launcher/api/resource?channel_id=1&key=6KcVuOkbcqjJomjZ&launcher_id=33&sub_channel_id=1",
};

interface CheckRes {
  remoteVersion: string;
  hasPreDownload: boolean
}

export const checkResource = async (key: GameKey) => {
  const url = staticResourceUrl[key];
  const checkRes: CheckRes = {
    remoteVersion: '',
    hasPreDownload: false
  }
  try {
    const { data } = await fetch<StaticResource>(url);
    checkRes.remoteVersion = data.data.game.latest.version;
    checkRes.hasPreDownload = data.data.pre_download_game != null;
  } catch (error) {
    console.log(error);
  }
  return checkRes
}
