
export default {
    data(){
        return{
            countParts:0,
            precent:0,
        }
    },
    methods: {
        setCountParts(event){
            // console.log(event);
            this.countParts = event.payload;
            // console.log(this.countParts);
        },
        setPrecent(event){
            // console.log(event);
            this.precent =  Math.ceil((event.payload.number_part / this.countParts) * 100);
            // console.log(this.precent);
        }
    },
    
}