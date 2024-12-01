#!/usr/bin/env python3

import datetime
import json
import asyncio
import aiohttp
import aiofiles
import pathlib
import certifi
import ssl
from zoneinfo import ZoneInfo

sslcontext = ssl.create_default_context(cafile=certifi.where())
async def main():
    with open("options.json") as f:
        options = json.load(f)
    year, session_id = options["year"], options["session_id"]

    today = datetime.datetime.now(ZoneInfo("America/New_York")).date()

    if year > today.year:
        raise Exception("Year is in the future")

    async def download_input_for_day(day: int, session: aiohttp.ClientSession):
        url = f"https://adventofcode.com/{year}/day/{day}/input"
        headers = {"Cookie": f"session={session_id}", "User-Agent": "github.com/Sp00ph/aoc2024"}
        path = pathlib.Path(f"input/day{day}.txt")
        if path.exists() or datetime.date(year, 12, day) > today:
            return
        async with session.get(url, headers=headers, ssl=sslcontext) as response:
            input = await response.text()
            async with aiofiles.open(path, "w") as f:
                await f.write(input)

    async with aiohttp.ClientSession() as session:
        await asyncio.gather(*[download_input_for_day(day, session) for day in range(1, 26)])

asyncio.run(main())
