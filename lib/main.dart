import 'package:flutter_btc_wallet/homepage.dart';
import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';

Future<void> main() async {
  // アプリ起動時に一度だけ取得する（起動中に変わることはない）
  final documentsDir = await getApplicationDocumentsDirectory();
  runApp(MyApp(documentsPath: documentsDir.path));
}

class MyApp extends StatelessWidget {
  const MyApp({super.key, required this.documentsPath});

  final String documentsPath;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Namer App',
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.green),
      ),
      home: HomePage(documentsPath: documentsPath),
    );
  }
}
