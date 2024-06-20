<script lang="ts" name="AppointButton">
  import { createEventDispatcher } from "svelte";
  import { KButton } from "@ikun-ui/button";
  import { HoyoClass } from "@/lib";

  export let needCheckConfig: boolean;

  export let hoyoClass: HoyoClass;

  const dispatch = createEventDispatcher<{
    "specify-exe": FileProp;
  }>();

  async function specifyExeFile() {
    const exeFile = needCheckConfig ? await hoyoClass.pickGameFile() : await hoyoClass.pickScriptFile();
    if (exeFile.path.length > 0 && exeFile.file.length > 0) {
      dispatch("specify-exe", exeFile);
    }
  }
</script>

<KButton type="error" size="md" on:click={specifyExeFile}>
  {needCheckConfig ? "2.指定游戏本体exe" : "3.获取游戏本体后自由选择exe脚本"}
</KButton>
