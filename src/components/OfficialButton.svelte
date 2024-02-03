<script lang="ts" name="OfficialButton">
  import { Invoker } from "@/enum/invoker";
  import { KButton } from "@ikun-ui/button";
  import { KMessage } from "@ikun-ui/message";
  import { invoke } from "@tauri-apps/api";

  export let dir: string;
  export let file: string;
  export let cls: string;

  async function openOfficialStarter() {
    const param: CheckPath = { dir, file };
    const flag = await invoke<boolean>(Invoker.open_exe, param);
    if (flag) {
      KMessage({
        type: "success",
        content: "官方下载器启动成功",
        duration: 1000,
      });
    } else {
      KMessage({
        type: "error",
        content: "官方下载器启动失败",
        duration: 1000,
      });
    }
  }
</script>

<KButton type="warning" size="md" {cls} on:click={openOfficialStarter}>
  官方下载器
</KButton>
