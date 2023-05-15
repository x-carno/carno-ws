import { reactive } from 'vue'

export const clientStore = reactive({
    count: 0,
    increment() {
        this.count++
    }
})