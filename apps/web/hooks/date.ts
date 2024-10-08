import { useDateFormat, useNow } from '@vueuse/core'
import { computed } from 'vue'

const year_zw = ['零', '一', '二', '三', '四', '五', '六', '七', '八', '九']
const month_zw = ['零', '正', '二', '三', '四', '五', '六', '七', '八', '九', '十', '十一', '十二']
const day_zw = ['零', '一', '二', '三', '四', '五', '六', '七', '八', '九', '十', '十一', '十二', '十三', '十四', '十五', '十六', '十七', '十八', '十九', '二十', '廿一', '廿二', '廿三', '廿四', '廿五', '廿六', '廿七', '廿八', '廿九', '三十']

export const useDays = () => {
    const date = useNow()

    const days = computed(() => {
        const _date = new Date(date.value)
        _date.setMonth(_date.getMonth() + 1, 0)
        return _date.getDate()
    })
    const oneDayWeek = computed(() => {
        const _date = new Date(date.value)
        _date.setDate(1)
        return _date.getDay()
    })

    const day = useDateFormat(date, 'DD')

    const month = useDateFormat(date, 'MM')

    const _hours = useDateFormat(date, 'HH')

    const hours = computed(() => {
        return _hours.value.split('')
    })

    const _minute = useDateFormat(date, 'mm')

    const minute = computed(() => {
        return _minute.value.split('')
    })

    const _second = useDateFormat(date, 'ss')

    const second = computed(() => {
        return _second.value.split('')
    })

    const week = useDateFormat(date, 'dddd')

    const ymd = computed(() => {
        const _date = new Date(date.value)

        const year = String(_date.getFullYear()).split('').map((item) => year_zw[Number(item) ?? 0]).join('') + '年'

        const month = month_zw[(_date.getMonth() + 1)] + '月'

        const day = day_zw[_date.getDate()] + '号'

        return [year, month, day]
    })

    return { days, oneDayWeek, day, month, minute, second, hours, week, ymd }
}