import { listen as tauriListen } from "@tauri-apps/api/event";
export default {
    data(){
        return{
            countParts:0,
            precent:0,
        }
    },
    methods: {
        setCountParts(event){
            this.countParts = event.payload + 1;
            console.log(this.countParts);
        },
        setPrecent(event){
            
            this.precent =  Math.ceil((event.payload.number_part / this.countParts) * 100);
            console.log(this.precent);
        }
    },
    async mounted(){
        await tauriListen("encode://count_parts", this.setCountParts);
        await tauriListen("encode://progress", this.setPrecent);
      },
}