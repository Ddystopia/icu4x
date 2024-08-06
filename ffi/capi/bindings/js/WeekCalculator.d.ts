// generated by diplomat-tool
import type { DataProvider } from "./DataProvider"
import type { IsoWeekday } from "./IsoWeekday"
import type { Locale } from "./Locale"
import type { WeekendContainsDay } from "./WeekendContainsDay"
import type { pointer, char } from "./diplomat-runtime.d.ts";


/** A Week calculator, useful to be passed in to `week_of_year()` on Date and DateTime types
*
*See the [Rust documentation for `WeekCalculator`](https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html) for more information.
*/
export class WeekCalculator {
    

    get ffiValue(): pointer;


    static create(provider: DataProvider, locale: Locale): WeekCalculator;

    static fromFirstDayOfWeekAndMinWeekDays(firstWeekday: IsoWeekday, minWeekDays: number): WeekCalculator;

    get firstWeekday(): IsoWeekday;

    get minWeekDays(): number;

    get weekend(): WeekendContainsDay;

    

}