import { open } from "@tauri-apps/api/dialog";
export default{
    data(){
        return{
            newDirectory: "",
        }
        
    },
    methods:{
        async newDirFile() {
            this.newDirectory = await open({
              directory: true,
              multiple: true,
              defaultPath: "/",
            });
          },
    },
    computed:{
        getNewDirectory() {
            try {
                if (!!this.newDirectory[0]){
                    return this.newDirectory[0] + `${'\\'}`;
                }
                else {
                    return this.newDirectory;
                }
            } catch {
              return this.newDirectory;
            }
          },
    }
}