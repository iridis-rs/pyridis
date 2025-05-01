import asyncio

from flarrow_api_python import sum_as_string, sleep, MyOtherClass

async def main():
    my_other_class = MyOtherClass(5)
    await my_other_class.print_after(1)

    my_other_class.set_value(10)
    await my_other_class.print_after(1)

    my_other_class.set_value(20)
    await my_other_class.print_after(1)

asyncio.run(main())
