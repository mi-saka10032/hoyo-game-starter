<script lang="ts" name="AppointButton">
  import { createEventDispatcher } from "svelte";
  import { KButton } from "@ikun-ui/button";
  import { FileProp, HoyoClass } from "@/lib";

  export let needCheckConfig: boolean;

  export let hoyoClass: HoyoClass;

  const dispatch = createEventDispatcher<{
    "specify-exe": FileProp;
  }>();

  async function specifyExeFile() {
    const exeFile = await hoyoClass.pickExeFile(needCheckConfig);
    if (exeFile.path.length > 0 && exeFile.file.length > 0) {
      dispatch("specify-exe", exeFile);
    }
  }
</script>

<KButton type="error" size="md" on:click={specifyExeFile}>
  指定启动文件({ needCheckConfig ? '需要游戏本体exe' : '自由选择exe' })
</KButton>
