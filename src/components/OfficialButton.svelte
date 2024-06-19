<script lang="ts" name="OfficialButton">
  import { KButton } from "@ikun-ui/button";
  import { KMessage } from "@ikun-ui/message";
  import { HoyoClass } from "@/lib";
  
  export let cls: string;

  export let hoyoClass: HoyoClass;

  async function openOfficialStarter() {
    const flag = await hoyoClass.openExeFile();
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
