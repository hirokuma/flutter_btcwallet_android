import 'package:flutter/material.dart';

class ErrorHelper {
  /// サッと知らせる SnackBar
  static void showSnackBar(
    BuildContext context,
    String message, {
    Color backgroundColor = Colors.red,
  }) {
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: backgroundColor,
        duration: const Duration(seconds: 3),
        behavior: SnackBarBehavior.floating, // 少し浮かせたスタイル（お好みで）
      ),
    );
  }

  /// しっかり確認させる AlertDialog
  static Future<void> showAlertDialog(
    BuildContext context, {
    String title = 'Error',
    required String message,
  }) {
    return showDialog<void>(
      context: context,
      builder: (context) {
        return AlertDialog(
          title: Text(title),
          content: Text(message),
          actions: [
            TextButton(
              onPressed: () => Navigator.of(context).pop(),
              child: const Text('OK'),
            ),
          ],
        );
      },
    );
  }
}
