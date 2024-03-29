import { NextRequest, NextResponse } from "next/server";

export async function middleware(request: NextRequest): Promise<NextResponse> {
   /**const cookies = request.cookies;

    const token = cookies.get("ttv_token");
    const client_id = cookies.get("ttv_client_id");

    let response = NextResponse.next();

    if (!token || !client_id) {
        response = NextResponse.rewrite(new URL("/login", request.url));
        response.cookies.set("redirect_url", request.url);
        return response;
    }

    if (cookies.get("ttv_moderated_channels") && cookies.get("ttv_moderating_index")) {
        return response;
    }

    const headers = {
        "Authorization": "Bearer " + token.value,
        "Client-Id": client_id.value
    };

    let r = await fetch(
        "https://api.twitch.tv/helix/users",
        {
            headers: headers
        }
    );

    let j = await r.json();
    let user = j.data[0];

    // maybe it can be done better
    response.cookies.set("twitch_user_data", JSON.stringify(user));

    let rr = await fetch(
        "https://api.twitch.tv/helix/moderation/channels?user_id=" + user.id,
        {
            headers: headers
        }
    );

    let jj = await rr.json();
    let channel_ids: any[] = jj.data;


    if (channel_ids.length !== 0) {
        const bot_response = await fetch("http://0.0.0.0:8085/v1/channels");
        const bot_json = await bot_response.json();
        const bot_channels: any[] = bot_json.data;

        let channels = [
            user.id
        ];

        // i dont care
        for (const channel_id of channel_ids) {
            for (const bot_channel of bot_channels) {
                if (bot_channel.alias_id == channel_id.broadcaster_id) {
                    channels.push(channel_id.broadcaster_id);
                }
            }
        }

        const users_response = await fetch(
            "https://api.twitch.tv/helix/users?id=" + channels.join("&id="),
            {
                headers: headers
            }
        );

        const users_json = await users_response.json();
        const users_data: any[] = users_json.data;

        const users_data_new = users_data.map((v) => {
            const x = bot_channels.find((y) => y.alias_id == v.id);

            v.internal_data = x;

            return v;
        });

        response.cookies.set("ttv_moderated_channels", JSON.stringify(users_data_new));
        response.cookies.set("ttv_moderating_index", "0");
    }

    return response;*/
    return NextResponse.next();
}

export const config = {
    matcher: ["/:path*"]
}