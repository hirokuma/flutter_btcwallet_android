import 'package:flutter_btc_wallet/homepage.dart';
import 'package:flutter_btc_wallet/src/rust/frb_generated.dart';

import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';

Future<void> main() async {
  // アプリ起動時に一度だけ取得する（起動中に変わることはない）
  await RustLib.init();
  final dataDir = await getApplicationSupportDirectory();
  runApp(MyApp(dataPath: dataDir.path));
}

class MyApp extends StatelessWidget {
  const MyApp({super.key, required this.dataPath});

  final String dataPath;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Namer App',
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.green),
      ),
      home: HomePage(dataPath: dataPath),
    );
  }
}
